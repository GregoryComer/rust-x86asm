use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaskmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 155952406, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 140, 12, 245, 22, 165, 75, 9], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 140, 35], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 140, 52, 195], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 140, 33], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectDisplaced(EDI, 377411636, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 142, 159, 52, 216, 126, 22], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1385577894, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 142, 12, 205, 166, 61, 150, 82], OperandSize::Qword)
}

#[test]
fn vpmaskmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 142, 3], OperandSize::Dword)
}

#[test]
fn vpmaskmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 142, 4, 134], OperandSize::Qword)
}

