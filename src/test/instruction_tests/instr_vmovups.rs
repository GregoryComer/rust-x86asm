use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovups_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 224], OperandSize::Dword)
}

#[test]
fn vmovups_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1457538395, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 180, 192, 91, 69, 224, 86], OperandSize::Dword)
}

#[test]
fn vmovups_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 232], OperandSize::Qword)
}

#[test]
fn vmovups_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 20, 112], OperandSize::Qword)
}

#[test]
fn vmovups_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 203], OperandSize::Dword)
}

#[test]
fn vmovups_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 4, 94], OperandSize::Dword)
}

#[test]
fn vmovups_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 228], OperandSize::Qword)
}

#[test]
fn vmovups_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 57], OperandSize::Qword)
}

#[test]
fn vmovups_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 16, 251], OperandSize::Dword)
}

#[test]
fn vmovups_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 979282305, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 16, 28, 221, 129, 169, 94, 58], OperandSize::Dword)
}

#[test]
fn vmovups_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 124, 137, 16, 243], OperandSize::Qword)
}

#[test]
fn vmovups_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM21)), operand2: Some(IndirectDisplaced(RAX, 1586657456, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 124, 143, 16, 168, 176, 120, 146, 94], OperandSize::Qword)
}

#[test]
fn vmovups_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 16, 227], OperandSize::Dword)
}

#[test]
fn vmovups_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 16, 4, 82], OperandSize::Dword)
}

#[test]
fn vmovups_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 124, 174, 16, 254], OperandSize::Qword)
}

#[test]
fn vmovups_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM11)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 124, 170, 16, 31], OperandSize::Qword)
}

#[test]
fn vmovups_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 16, 227], OperandSize::Dword)
}

#[test]
fn vmovups_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 16, 20, 129], OperandSize::Dword)
}

#[test]
fn vmovups_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 16, 199], OperandSize::Qword)
}

#[test]
fn vmovups_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 734481360, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 202, 16, 156, 122, 208, 75, 199, 43], OperandSize::Qword)
}

#[test]
fn vmovups_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 226], OperandSize::Dword)
}

#[test]
fn vmovups_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1407751316, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 12, 125, 148, 148, 232, 83], OperandSize::Dword)
}

#[test]
fn vmovups_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 204], OperandSize::Qword)
}

#[test]
fn vmovups_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 36, 73], OperandSize::Qword)
}

#[test]
fn vmovups_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 221], OperandSize::Dword)
}

#[test]
fn vmovups_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(EDX, 417064710, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 178, 6, 231, 219, 24], OperandSize::Dword)
}

#[test]
fn vmovups_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 215], OperandSize::Qword)
}

#[test]
fn vmovups_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(RDI, 1604146167, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 167, 247, 83, 157, 95], OperandSize::Qword)
}

#[test]
fn vmovups_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 16, 240], OperandSize::Dword)
}

#[test]
fn vmovups_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledDisplaced(EDI, Four, 551208855, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 36, 189, 151, 199, 218, 32], OperandSize::Dword)
}

#[test]
fn vmovups_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 124, 143, 16, 219], OperandSize::Qword)
}

#[test]
fn vmovups_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1021407832, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 17, 188, 219, 88, 114, 225, 60], OperandSize::Qword)
}

#[test]
fn vmovups_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 171, 16, 207], OperandSize::Dword)
}

#[test]
fn vmovups_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1887869282, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 60, 93, 98, 153, 134, 112], OperandSize::Dword)
}

#[test]
fn vmovups_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 124, 175, 16, 254], OperandSize::Qword)
}

#[test]
fn vmovups_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 414028346, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 40, 17, 180, 113, 58, 146, 173, 24], OperandSize::Qword)
}

#[test]
fn vmovups_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 203, 16, 253], OperandSize::Dword)
}

#[test]
fn vmovups_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1128650136, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 17, 12, 117, 152, 213, 69, 67], OperandSize::Dword)
}

#[test]
fn vmovups_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 124, 202, 16, 236], OperandSize::Qword)
}

#[test]
fn vmovups_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 17, 55], OperandSize::Qword)
}

