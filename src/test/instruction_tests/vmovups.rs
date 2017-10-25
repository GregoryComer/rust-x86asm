use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovups_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 195], OperandSize::Dword)
}

fn vmovups_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 2140322767, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 36, 125, 207, 187, 146, 127], OperandSize::Dword)
}

fn vmovups_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 232], OperandSize::Qword)
}

fn vmovups_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 20, 126], OperandSize::Qword)
}

fn vmovups_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 230], OperandSize::Dword)
}

fn vmovups_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1730926370, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 60, 157, 34, 215, 43, 103], OperandSize::Dword)
}

fn vmovups_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 192], OperandSize::Qword)
}

fn vmovups_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(RAX, 1348036129, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 136, 33, 102, 89, 80], OperandSize::Qword)
}

fn vmovups_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 16, 232], OperandSize::Dword)
}

fn vmovups_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 16, 20, 250], OperandSize::Dword)
}

fn vmovups_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 124, 143, 16, 195], OperandSize::Qword)
}

fn vmovups_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM23)), operand2: Some(IndirectDisplaced(RDI, 1821827039, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 124, 139, 16, 191, 223, 223, 150, 108], OperandSize::Qword)
}

fn vmovups_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 173, 16, 246], OperandSize::Dword)
}

fn vmovups_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 477083738, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 171, 16, 52, 221, 90, 184, 111, 28], OperandSize::Dword)
}

fn vmovups_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 124, 174, 16, 195], OperandSize::Qword)
}

fn vmovups_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM11)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 124, 171, 16, 28, 78], OperandSize::Qword)
}

fn vmovups_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 202, 16, 217], OperandSize::Dword)
}

fn vmovups_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 389211989, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 202, 16, 44, 149, 85, 231, 50, 23], OperandSize::Dword)
}

fn vmovups_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 124, 201, 16, 242], OperandSize::Qword)
}

fn vmovups_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 324130879, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 124, 203, 16, 20, 213, 63, 216, 81, 19], OperandSize::Qword)
}

fn vmovups_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 247], OperandSize::Dword)
}

fn vmovups_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledDisplaced(ECX, Two, 82381614, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 12, 77, 46, 11, 233, 4], OperandSize::Dword)
}

fn vmovups_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 16, 230], OperandSize::Qword)
}

fn vmovups_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 20, 138], OperandSize::Qword)
}

fn vmovups_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 218], OperandSize::Dword)
}

fn vmovups_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 19], OperandSize::Dword)
}

fn vmovups_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 16, 244], OperandSize::Qword)
}

fn vmovups_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(RCX, 69390963, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 145, 115, 210, 34, 4], OperandSize::Qword)
}

fn vmovups_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 16, 244], OperandSize::Dword)
}

fn vmovups_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 17, 54], OperandSize::Dword)
}

fn vmovups_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 124, 142, 16, 231], OperandSize::Qword)
}

fn vmovups_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 8, 17, 36, 193], OperandSize::Qword)
}

fn vmovups_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 16, 212], OperandSize::Dword)
}

fn vmovups_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 17, 36, 118], OperandSize::Dword)
}

fn vmovups_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 124, 173, 16, 237], OperandSize::Qword)
}

fn vmovups_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(IndirectDisplaced(RCX, 1064404646, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 40, 17, 169, 166, 134, 113, 63], OperandSize::Qword)
}

fn vmovups_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 16, 218], OperandSize::Dword)
}

fn vmovups_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 17, 1], OperandSize::Dword)
}

fn vmovups_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 33, 124, 202, 16, 234], OperandSize::Qword)
}

fn vmovups_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPS, operand1: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 72, 17, 34], OperandSize::Qword)
}

