use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pinsrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 245, 106], OperandSize::Dword)
}

fn pinsrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1749524261, Some(OperandSize::Byte), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 44, 117, 37, 159, 71, 104, 109], OperandSize::Dword)
}

fn pinsrb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 239, 12], OperandSize::Qword)
}

fn pinsrb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1269005607, Some(OperandSize::Byte), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 4, 189, 39, 125, 163, 75, 38], OperandSize::Qword)
}

