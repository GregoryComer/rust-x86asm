use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vroundps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 201, 53], OperandSize::Dword)
}

#[test]
fn vroundps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 548875255, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 172, 112, 247, 43, 183, 32, 80], OperandSize::Dword)
}

#[test]
fn vroundps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 254, 24], OperandSize::Qword)
}

#[test]
fn vroundps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1133341249, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 8, 4, 93, 65, 106, 141, 67, 49], OperandSize::Qword)
}

#[test]
fn vroundps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 193, 1], OperandSize::Dword)
}

#[test]
fn vroundps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EBX, 1254447755, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 155, 139, 90, 197, 74, 79], OperandSize::Dword)
}

#[test]
fn vroundps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 216, 8], OperandSize::Qword)
}

#[test]
fn vroundps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VROUNDPS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 8, 52, 70, 44], OperandSize::Qword)
}

