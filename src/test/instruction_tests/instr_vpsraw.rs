use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsraw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 113, 224, 28], OperandSize::Dword)
}

#[test]
fn vpsraw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 113, 225, 66], OperandSize::Qword)
}

#[test]
fn vpsraw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 113, 228, 107], OperandSize::Dword)
}

#[test]
fn vpsraw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 113, 230, 57], OperandSize::Qword)
}

#[test]
fn vpsraw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 142, 113, 230, 31], OperandSize::Dword)
}

#[test]
fn vpsraw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 113, 39, 79], OperandSize::Dword)
}

#[test]
fn vpsraw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM20)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 125, 137, 113, 228, 72], OperandSize::Qword)
}

#[test]
fn vpsraw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1824624942, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 142, 113, 36, 117, 46, 145, 193, 108, 117], OperandSize::Qword)
}

#[test]
fn vpsraw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 172, 113, 229, 75], OperandSize::Dword)
}

#[test]
fn vpsraw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1457562091, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 113, 164, 183, 235, 161, 224, 86, 54], OperandSize::Dword)
}

#[test]
fn vpsraw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 53, 166, 113, 230, 24], OperandSize::Qword)
}

#[test]
fn vpsraw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM12)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 29, 169, 113, 34, 24], OperandSize::Qword)
}

#[test]
fn vpsraw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 203, 113, 224, 82], OperandSize::Dword)
}

#[test]
fn vpsraw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1983525444, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 203, 113, 164, 130, 68, 50, 58, 118, 1], OperandSize::Dword)
}

#[test]
fn vpsraw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM15)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 109, 199, 113, 231, 56], OperandSize::Qword)
}

#[test]
fn vpsraw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 543962089, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 207, 113, 36, 197, 233, 51, 108, 32, 100], OperandSize::Qword)
}

#[test]
fn vpsraw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 225, 204], OperandSize::Dword)
}

#[test]
fn vpsraw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1843940469, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 225, 156, 66, 117, 76, 232, 109], OperandSize::Dword)
}

#[test]
fn vpsraw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 225, 216], OperandSize::Qword)
}

#[test]
fn vpsraw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 589451063, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 225, 52, 133, 55, 79, 34, 35], OperandSize::Qword)
}

#[test]
fn vpsraw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 225, 205], OperandSize::Dword)
}

#[test]
fn vpsraw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 967034471, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 225, 188, 203, 103, 198, 163, 57], OperandSize::Dword)
}

#[test]
fn vpsraw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 225, 241], OperandSize::Qword)
}

#[test]
fn vpsraw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 225, 12, 123], OperandSize::Qword)
}

#[test]
fn vpsraw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 143, 225, 194], OperandSize::Dword)
}

#[test]
fn vpsraw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 139, 225, 28, 142], OperandSize::Dword)
}

#[test]
fn vpsraw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 21, 129, 225, 253], OperandSize::Qword)
}

#[test]
fn vpsraw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 61, 131, 225, 4, 137], OperandSize::Qword)
}

#[test]
fn vpsraw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 173, 225, 237], OperandSize::Dword)
}

#[test]
fn vpsraw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 830123722, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 175, 225, 138, 202, 174, 122, 49], OperandSize::Dword)
}

#[test]
fn vpsraw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 77, 171, 225, 211], OperandSize::Qword)
}

#[test]
fn vpsraw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectDisplaced(RCX, 311121515, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 101, 164, 225, 169, 107, 86, 139, 18], OperandSize::Qword)
}

#[test]
fn vpsraw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 205, 225, 251], OperandSize::Dword)
}

#[test]
fn vpsraw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 462674269, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 202, 225, 60, 181, 93, 217, 147, 27], OperandSize::Dword)
}

#[test]
fn vpsraw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 21, 194, 225, 215], OperandSize::Qword)
}

#[test]
fn vpsraw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RBX, 1014490416, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 37, 205, 225, 155, 48, 229, 119, 60], OperandSize::Qword)
}

