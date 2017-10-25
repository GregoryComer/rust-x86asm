use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovupd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 215], OperandSize::Dword)
}

fn vmovupd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 482993199, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 172, 187, 47, 228, 201, 28], OperandSize::Dword)
}

fn vmovupd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 241], OperandSize::Qword)
}

fn vmovupd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 32], OperandSize::Qword)
}

fn vmovupd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 204], OperandSize::Dword)
}

fn vmovupd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 790862239, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 36, 69, 159, 153, 35, 47], OperandSize::Dword)
}

fn vmovupd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 210], OperandSize::Qword)
}

fn vmovupd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 4, 119], OperandSize::Qword)
}

fn vmovupd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 16, 229], OperandSize::Dword)
}

fn vmovupd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1815393563, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 16, 180, 88, 27, 181, 52, 108], OperandSize::Dword)
}

fn vmovupd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 253, 137, 16, 218], OperandSize::Qword)
}

fn vmovupd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 851303252, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 253, 139, 16, 52, 213, 84, 219, 189, 50], OperandSize::Qword)
}

fn vmovupd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 16, 228], OperandSize::Dword)
}

fn vmovupd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 16, 18], OperandSize::Dword)
}

fn vmovupd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 253, 169, 16, 228], OperandSize::Qword)
}

fn vmovupd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM25)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1279962311, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 253, 171, 16, 12, 125, 199, 172, 74, 76], OperandSize::Qword)
}

fn vmovupd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 16, 238], OperandSize::Dword)
}

fn vmovupd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 398401863, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 16, 36, 245, 71, 33, 191, 23], OperandSize::Dword)
}

fn vmovupd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 253, 206, 16, 251], OperandSize::Qword)
}

fn vmovupd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM20)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 205, 16, 38], OperandSize::Qword)
}

fn vmovupd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 254], OperandSize::Dword)
}

fn vmovupd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 52, 134], OperandSize::Dword)
}

fn vmovupd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 224], OperandSize::Qword)
}

fn vmovupd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 916676229, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 148, 192, 133, 94, 163, 54], OperandSize::Qword)
}

fn vmovupd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 227], OperandSize::Dword)
}

fn vmovupd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 46068064, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 60, 205, 96, 241, 190, 2], OperandSize::Dword)
}

fn vmovupd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 241], OperandSize::Qword)
}

fn vmovupd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 24], OperandSize::Qword)
}

fn vmovupd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 16, 241], OperandSize::Dword)
}

fn vmovupd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1462422177, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 12, 117, 161, 202, 42, 87], OperandSize::Dword)
}

fn vmovupd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 253, 142, 16, 204], OperandSize::Qword)
}

fn vmovupd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 109169535, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 17, 188, 142, 127, 203, 129, 6], OperandSize::Qword)
}

fn vmovupd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 16, 244], OperandSize::Dword)
}

fn vmovupd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 2078533097, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 180, 218, 233, 229, 227, 123], OperandSize::Dword)
}

fn vmovupd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 253, 171, 16, 243], OperandSize::Qword)
}

fn vmovupd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(RDX, 1088890470, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 40, 17, 146, 102, 38, 231, 64], OperandSize::Qword)
}

fn vmovupd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 202, 16, 218], OperandSize::Dword)
}

fn vmovupd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 17, 20, 129], OperandSize::Dword)
}

fn vmovupd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 253, 207, 16, 232], OperandSize::Qword)
}

fn vmovupd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 72, 17, 4, 153], OperandSize::Qword)
}

