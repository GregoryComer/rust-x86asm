use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn orpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 250], OperandSize::Dword)
}

fn orpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 187432429, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 52, 253, 237, 253, 43, 11], OperandSize::Dword)
}

fn orpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 249], OperandSize::Qword)
}

fn orpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 424517837, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 20, 149, 205, 160, 77, 25], OperandSize::Qword)
}

