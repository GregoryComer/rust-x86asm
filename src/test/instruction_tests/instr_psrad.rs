use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM3)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 227, 40], OperandSize::Dword)
}

#[test]
fn psrad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM7)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 231, 116], OperandSize::Qword)
}

#[test]
fn psrad_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM0)), operand2: Some(Literal8(111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 224, 111], OperandSize::Dword)
}

#[test]
fn psrad_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 226, 78], OperandSize::Qword)
}

#[test]
fn psrad_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 212], OperandSize::Dword)
}

#[test]
fn psrad_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1194974301, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 60, 125, 93, 220, 57, 71], OperandSize::Dword)
}

#[test]
fn psrad_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 223], OperandSize::Qword)
}

#[test]
fn psrad_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 52, 222], OperandSize::Qword)
}

#[test]
fn psrad_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 236], OperandSize::Dword)
}

#[test]
fn psrad_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1064821475, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 4, 245, 227, 226, 119, 63], OperandSize::Dword)
}

#[test]
fn psrad_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 221], OperandSize::Qword)
}

#[test]
fn psrad_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 30], OperandSize::Qword)
}

