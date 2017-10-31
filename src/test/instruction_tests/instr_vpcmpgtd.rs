use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 102, 228], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ESI, 812918292, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 102, 142, 20, 38, 116, 48], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 102, 218], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 102, 9], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 102, 214], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 102, 33], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 102, 251], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 102, 1], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 12, 102, 201], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 12, 102, 36, 153], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 25, 102, 44, 130], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 93, 15, 102, 253], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 12, 102, 48], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 190985899, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 22, 102, 36, 221, 171, 54, 98, 11], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 44, 102, 232], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1158479853, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 44, 102, 28, 93, 237, 255, 12, 69], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 520827933, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 58, 102, 180, 176, 29, 52, 11, 31], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 29, 38, 102, 247], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1371113667, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 37, 41, 102, 140, 146, 195, 136, 185, 81], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RDI, 1677685908, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 53, 50, 102, 183, 148, 116, 255, 99], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 75, 102, 255], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 1390126947, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 76, 102, 180, 142, 99, 167, 219, 82], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 540042118, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 69, 95, 102, 180, 216, 134, 99, 48, 32], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 101, 69, 102, 238], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RDX, 173763872, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 37, 70, 102, 170, 32, 109, 91, 10], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1238168274, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 61, 94, 102, 20, 197, 210, 242, 204, 73], OperandSize::Qword)
}

