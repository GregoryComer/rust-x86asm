use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovups_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 212], OperandSize::Dword)
}

#[test]
fn vmovups_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1203195761, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 28, 253, 113, 79, 183, 71], OperandSize::Dword)
}

#[test]
fn vmovups_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 227], OperandSize::Qword)
}

#[test]
fn vmovups_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1803278933, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 4, 197, 85, 218, 123, 107], OperandSize::Qword)
}

#[test]
fn vmovups_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 233], OperandSize::Dword)
}

#[test]
fn vmovups_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 36, 115], OperandSize::Dword)
}

#[test]
fn vmovups_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 229], OperandSize::Qword)
}

#[test]
fn vmovups_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 52, 123], OperandSize::Qword)
}

#[test]
fn vmovups_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 140, 16, 214], OperandSize::Dword)
}

#[test]
fn vmovups_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 975607194, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 16, 180, 146, 154, 149, 38, 58], OperandSize::Dword)
}

#[test]
fn vmovups_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 124, 138, 16, 243], OperandSize::Qword)
}

#[test]
fn vmovups_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 124, 140, 16, 12, 94], OperandSize::Qword)
}

#[test]
fn vmovups_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 16, 239], OperandSize::Dword)
}

#[test]
fn vmovups_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 387968455, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 16, 60, 125, 199, 237, 31, 23], OperandSize::Dword)
}

#[test]
fn vmovups_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 124, 171, 16, 220], OperandSize::Qword)
}

#[test]
fn vmovups_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 2053404631, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 124, 175, 16, 180, 67, 215, 119, 100, 122], OperandSize::Qword)
}

#[test]
fn vmovups_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 205, 16, 204], OperandSize::Dword)
}

#[test]
fn vmovups_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 205, 16, 14], OperandSize::Dword)
}

#[test]
fn vmovups_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 124, 202, 16, 240], OperandSize::Qword)
}

#[test]
fn vmovups_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectDisplaced(RCX, 446297406, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 124, 202, 16, 185, 62, 245, 153, 26], OperandSize::Qword)
}

#[test]
fn vmovups_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 192], OperandSize::Dword)
}

#[test]
fn vmovups_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 42], OperandSize::Dword)
}

#[test]
fn vmovups_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 236], OperandSize::Qword)
}

#[test]
fn vmovups_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 189789121, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 140, 155, 193, 243, 79, 11], OperandSize::Qword)
}

#[test]
fn vmovups_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 255], OperandSize::Dword)
}

#[test]
fn vmovups_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 44, 199], OperandSize::Dword)
}

#[test]
fn vmovups_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 208], OperandSize::Qword)
}

#[test]
fn vmovups_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(RSI, 91443190, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 190, 246, 79, 115, 5], OperandSize::Qword)
}

#[test]
fn vmovups_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 16, 217], OperandSize::Dword)
}

#[test]
fn vmovups_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1128590534, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 52, 69, 198, 236, 68, 67], OperandSize::Dword)
}

#[test]
fn vmovups_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 124, 138, 16, 250], OperandSize::Qword)
}

#[test]
fn vmovups_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(RBX, 676699091, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 17, 155, 211, 155, 85, 40], OperandSize::Qword)
}

#[test]
fn vmovups_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 16, 220], OperandSize::Dword)
}

#[test]
fn vmovups_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 1966668209, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 132, 145, 177, 249, 56, 117], OperandSize::Dword)
}

#[test]
fn vmovups_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 124, 171, 16, 243], OperandSize::Qword)
}

#[test]
fn vmovups_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 28, 192], OperandSize::Qword)
}

#[test]
fn vmovups_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 202, 16, 233], OperandSize::Dword)
}

#[test]
fn vmovups_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(EDI, 572259978, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 17, 167, 138, 254, 27, 34], OperandSize::Dword)
}

#[test]
fn vmovups_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 124, 205, 16, 252], OperandSize::Qword)
}

#[test]
fn vmovups_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 2009472932, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 17, 132, 80, 164, 31, 198, 119], OperandSize::Qword)
}

