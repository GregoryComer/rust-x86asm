use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsraw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 113, 230, 124], OperandSize::Dword)
}

#[test]
fn vpsraw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 113, 231, 18], OperandSize::Qword)
}

#[test]
fn vpsraw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 113, 227, 30], OperandSize::Dword)
}

#[test]
fn vpsraw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 113, 229, 26], OperandSize::Qword)
}

#[test]
fn vpsraw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 113, 228, 23], OperandSize::Dword)
}

#[test]
fn vpsraw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 113, 39, 45], OperandSize::Dword)
}

#[test]
fn vpsraw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM19)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 69, 142, 113, 227, 92], OperandSize::Qword)
}

#[test]
fn vpsraw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 3414763, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 130, 113, 36, 221, 235, 26, 52, 0, 24], OperandSize::Qword)
}

#[test]
fn vpsraw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 174, 113, 231, 63], OperandSize::Dword)
}

#[test]
fn vpsraw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 113, 35, 50], OperandSize::Dword)
}

#[test]
fn vpsraw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 171, 113, 230, 47], OperandSize::Qword)
}

#[test]
fn vpsraw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM12)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 29, 171, 113, 33, 5], OperandSize::Qword)
}

#[test]
fn vpsraw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 203, 113, 225, 94], OperandSize::Dword)
}

#[test]
fn vpsraw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 993860148, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 205, 113, 36, 77, 52, 26, 61, 59, 36], OperandSize::Dword)
}

#[test]
fn vpsraw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM19)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 53, 197, 113, 227, 16], OperandSize::Qword)
}

#[test]
fn vpsraw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 195, 113, 36, 145, 116], OperandSize::Qword)
}

#[test]
fn vpsraw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 225, 242], OperandSize::Dword)
}

#[test]
fn vpsraw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 122398562, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 225, 20, 189, 98, 167, 75, 7], OperandSize::Dword)
}

#[test]
fn vpsraw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 225, 239], OperandSize::Qword)
}

#[test]
fn vpsraw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1658689163, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 225, 44, 245, 139, 150, 221, 98], OperandSize::Qword)
}

#[test]
fn vpsraw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 225, 202], OperandSize::Dword)
}

#[test]
fn vpsraw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1774755435, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 225, 44, 149, 107, 158, 200, 105], OperandSize::Dword)
}

#[test]
fn vpsraw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 225, 237], OperandSize::Qword)
}

#[test]
fn vpsraw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 45494905, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 225, 180, 216, 121, 50, 182, 2], OperandSize::Qword)
}

#[test]
fn vpsraw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 143, 225, 192], OperandSize::Dword)
}

#[test]
fn vpsraw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 885440846, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 225, 139, 78, 193, 198, 52], OperandSize::Dword)
}

#[test]
fn vpsraw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 29, 129, 225, 208], OperandSize::Qword)
}

#[test]
fn vpsraw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 176496949, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 13, 131, 225, 28, 125, 53, 33, 133, 10], OperandSize::Qword)
}

#[test]
fn vpsraw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 171, 225, 246], OperandSize::Dword)
}

#[test]
fn vpsraw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1587137231, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 170, 225, 36, 189, 207, 202, 153, 94], OperandSize::Dword)
}

#[test]
fn vpsraw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 53, 164, 225, 228], OperandSize::Qword)
}

#[test]
fn vpsraw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectDisplaced(RDX, 1455903073, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 29, 173, 225, 178, 97, 81, 199, 86], OperandSize::Qword)
}

#[test]
fn vpsraw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 205, 225, 226], OperandSize::Dword)
}

#[test]
fn vpsraw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 202, 225, 38], OperandSize::Dword)
}

#[test]
fn vpsraw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 117, 206, 225, 195], OperandSize::Qword)
}

#[test]
fn vpsraw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAW, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectDisplaced(RSI, 2144640661, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 77, 194, 225, 182, 149, 158, 212, 127], OperandSize::Qword)
}

