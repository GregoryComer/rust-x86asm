use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 246, 9, 38, 235], OperandSize::Dword)
}

#[test]
fn vptestnmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EBX, 1669049034, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 222, 15, 38, 179, 202, 170, 123, 99], OperandSize::Dword)
}

#[test]
fn vptestnmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 222, 5, 38, 221], OperandSize::Qword)
}

#[test]
fn vptestnmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectDisplaced(RDX, 189729090, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 206, 3, 38, 178, 66, 9, 79, 11], OperandSize::Qword)
}

#[test]
fn vptestnmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 198, 47, 38, 248], OperandSize::Dword)
}

#[test]
fn vptestnmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ECX, 201168879, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 198, 47, 38, 137, 239, 151, 253, 11], OperandSize::Dword)
}

#[test]
fn vptestnmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 198, 33, 38, 204], OperandSize::Qword)
}

#[test]
fn vptestnmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 263205186, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 222, 37, 38, 44, 205, 66, 49, 176, 15], OperandSize::Qword)
}

#[test]
fn vptestnmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 230, 79, 38, 223], OperandSize::Dword)
}

#[test]
fn vptestnmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 238, 76, 38, 26], OperandSize::Dword)
}

#[test]
fn vptestnmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 206, 71, 38, 219], OperandSize::Qword)
}

#[test]
fn vptestnmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RCX, 215605181, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 134, 66, 38, 145, 189, 223, 217, 12], OperandSize::Qword)
}

