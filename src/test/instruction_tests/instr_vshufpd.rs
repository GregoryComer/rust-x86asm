use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 198, 219, 81], OperandSize::Dword)
}

#[test]
fn vshufpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 198, 12, 183, 124], OperandSize::Dword)
}

#[test]
fn vshufpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 198, 199, 56], OperandSize::Qword)
}

#[test]
fn vshufpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 198, 52, 80, 84], OperandSize::Qword)
}

#[test]
fn vshufpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 198, 217, 116], OperandSize::Dword)
}

#[test]
fn vshufpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 198, 56, 125], OperandSize::Dword)
}

#[test]
fn vshufpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 198, 238, 74], OperandSize::Qword)
}

#[test]
fn vshufpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 198, 4, 178, 68], OperandSize::Qword)
}

#[test]
fn vshufpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 140, 198, 233, 7], OperandSize::Dword)
}

#[test]
fn vshufpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1445871992, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 140, 198, 36, 213, 120, 65, 46, 86, 116], OperandSize::Dword)
}

#[test]
fn vshufpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 1913201351, Some(OperandSize::Qword), None)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 158, 198, 167, 199, 34, 9, 114, 35], OperandSize::Dword)
}

#[test]
fn vshufpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM23)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 49, 221, 133, 198, 223, 20], OperandSize::Qword)
}

#[test]
fn vshufpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1693603066, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 189, 143, 198, 52, 93, 250, 84, 242, 100, 65], OperandSize::Qword)
}

#[test]
fn vshufpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 221, 156, 198, 2, 105], OperandSize::Qword)
}

#[test]
fn vshufpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 175, 198, 221, 53], OperandSize::Dword)
}

#[test]
fn vshufpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 391066618, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 175, 198, 180, 202, 250, 51, 79, 23, 70], OperandSize::Dword)
}

#[test]
fn vshufpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 187, 198, 48, 79], OperandSize::Dword)
}

#[test]
fn vshufpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 133, 172, 198, 216, 82], OperandSize::Qword)
}

#[test]
fn vshufpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 245, 163, 198, 52, 152, 122], OperandSize::Qword)
}

#[test]
fn vshufpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 160780973, Some(OperandSize::Qword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 173, 180, 198, 52, 133, 173, 82, 149, 9, 13], OperandSize::Qword)
}

#[test]
fn vshufpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 202, 198, 240, 74], OperandSize::Dword)
}

#[test]
fn vshufpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 529943064, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 206, 198, 188, 142, 24, 74, 150, 31, 40], OperandSize::Dword)
}

#[test]
fn vshufpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1486905360, Some(OperandSize::Qword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 222, 198, 132, 83, 16, 96, 160, 88, 68], OperandSize::Dword)
}

#[test]
fn vshufpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM22)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 213, 195, 198, 246, 105], OperandSize::Qword)
}

#[test]
fn vshufpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 1857561530, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 141, 197, 198, 148, 112, 186, 35, 184, 110, 108], OperandSize::Qword)
}

#[test]
fn vshufpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(RSI, 960840891, Some(OperandSize::Qword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 217, 198, 174, 187, 68, 69, 57, 113], OperandSize::Qword)
}

