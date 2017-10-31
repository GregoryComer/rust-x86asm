use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 78, 240], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDX, 1513700928, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 78, 138, 64, 62, 57, 90], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 2127983084, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 153, 78, 136, 236, 113, 214, 126], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 137, 78, 253], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1285077402, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 138, 78, 132, 247, 154, 185, 152, 76], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 155, 78, 52, 200], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 78, 192], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1662832580, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 78, 12, 157, 196, 207, 28, 99], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 2110768661, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 78, 4, 245, 21, 198, 207, 125], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 125, 174, 78, 224], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1623831116, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 169, 78, 188, 154, 76, 178, 201, 96], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM20)), operand2: Some(IndirectDisplaced(RDI, 2092857357, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 190, 78, 167, 13, 120, 190, 124], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 78, 249], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 78, 12, 91], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EDI, 159059514, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 223, 78, 175, 58, 14, 123, 9], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 204, 78, 234], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 781327374, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 207, 78, 36, 253, 14, 28, 146, 46], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1602558464, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 78, 20, 245, 0, 26, 133, 95], OperandSize::Qword)
}

