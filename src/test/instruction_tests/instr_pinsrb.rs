use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 195, 70], OperandSize::Dword)
}

#[test]
fn pinsrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1527507085, Some(OperandSize::Byte), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 148, 190, 141, 232, 11, 91, 46], OperandSize::Dword)
}

#[test]
fn pinsrb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 236, 111], OperandSize::Qword)
}

#[test]
fn pinsrb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RSI, 354762681, Some(OperandSize::Byte), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 142, 185, 63, 37, 21, 94], OperandSize::Qword)
}

