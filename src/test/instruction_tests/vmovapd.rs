use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovapd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 250], OperandSize::Dword)
}

fn vmovapd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 1672154693, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 182, 69, 14, 171, 99], OperandSize::Dword)
}

fn vmovapd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 251], OperandSize::Qword)
}

fn vmovapd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 36, 147], OperandSize::Qword)
}

fn vmovapd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 197], OperandSize::Dword)
}

fn vmovapd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1693769309, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 44, 85, 93, 222, 244, 100], OperandSize::Dword)
}

fn vmovapd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 213], OperandSize::Qword)
}

fn vmovapd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(RDX, 373749987, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 154, 227, 248, 70, 22], OperandSize::Qword)
}

fn vmovapd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 40, 252], OperandSize::Dword)
}

fn vmovapd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 40, 36, 115], OperandSize::Dword)
}

fn vmovapd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 253, 142, 40, 219], OperandSize::Qword)
}

fn vmovapd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 138, 40, 28, 154], OperandSize::Qword)
}

fn vmovapd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 40, 239], OperandSize::Dword)
}

fn vmovapd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 40, 36, 194], OperandSize::Dword)
}

fn vmovapd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 253, 174, 40, 250], OperandSize::Qword)
}

fn vmovapd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM17)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 173, 40, 10], OperandSize::Qword)
}

fn vmovapd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 40, 222], OperandSize::Dword)
}

fn vmovapd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 205, 40, 4, 87], OperandSize::Dword)
}

fn vmovapd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 253, 204, 40, 228], OperandSize::Qword)
}

fn vmovapd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 202, 40, 4, 255], OperandSize::Qword)
}

fn vmovapd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 200], OperandSize::Dword)
}

fn vmovapd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 27], OperandSize::Dword)
}

fn vmovapd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 209], OperandSize::Qword)
}

fn vmovapd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1532567434, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 36, 77, 138, 31, 89, 91], OperandSize::Qword)
}

fn vmovapd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 202], OperandSize::Dword)
}

fn vmovapd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 20, 120], OperandSize::Dword)
}

fn vmovapd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 245], OperandSize::Qword)
}

fn vmovapd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(RAX, 315493641, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 152, 9, 13, 206, 18], OperandSize::Qword)
}

fn vmovapd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 40, 210], OperandSize::Dword)
}

fn vmovapd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 60, 119], OperandSize::Dword)
}

fn vmovapd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 253, 143, 40, 234], OperandSize::Qword)
}

fn vmovapd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1289660891, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 41, 12, 77, 219, 169, 222, 76], OperandSize::Qword)
}

fn vmovapd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 40, 251], OperandSize::Dword)
}

fn vmovapd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 1], OperandSize::Dword)
}

fn vmovapd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 17, 253, 170, 40, 241], OperandSize::Qword)
}

fn vmovapd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1091287069, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 20, 213, 29, 184, 11, 65], OperandSize::Qword)
}

fn vmovapd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 40, 231], OperandSize::Dword)
}

fn vmovapd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 1915686470, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 41, 156, 198, 70, 14, 47, 114], OperandSize::Dword)
}

fn vmovapd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 253, 203, 40, 228], OperandSize::Qword)
}

fn vmovapd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(RDX, 2109222964, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 72, 41, 170, 52, 48, 184, 125], OperandSize::Qword)
}

