use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 239], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 939076575, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 140, 208, 223, 43, 249, 55], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 229], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 25], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 230], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 48], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 207], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 27], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 91, 193], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 883977601, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 91, 52, 253, 129, 109, 176, 52], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 125, 142, 91, 208], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 569522994, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 125, 137, 91, 140, 153, 50, 59, 242, 33], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 91, 222], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1423674877, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 91, 20, 213, 253, 141, 219, 84], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 91, 248], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM22)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 125, 169, 91, 55], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 249, 91, 222], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(ECX, 1256982500, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 91, 145, 228, 7, 236, 74], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 125, 253, 91, 255], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 91, 18], OperandSize::Qword)
}

