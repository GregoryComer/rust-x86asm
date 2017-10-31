use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 245, 4], OperandSize::Dword)
}

#[test]
fn pinsrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 31, 102], OperandSize::Dword)
}

#[test]
fn pinsrb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 193, 83], OperandSize::Qword)
}

#[test]
fn pinsrb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1730105712, Some(OperandSize::Byte), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 36, 85, 112, 81, 31, 103, 44], OperandSize::Qword)
}

