use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 243, 14], OperandSize::Dword)
}

#[test]
fn pinsrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Byte), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 60, 254, 44], OperandSize::Dword)
}

#[test]
fn pinsrb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 255, 115], OperandSize::Qword)
}

#[test]
fn pinsrb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDX, 1950746331, Some(OperandSize::Byte), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 146, 219, 6, 70, 116, 67], OperandSize::Qword)
}

