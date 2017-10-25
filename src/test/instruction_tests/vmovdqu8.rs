use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovdqu8_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 138, 111, 211], OperandSize::Dword)
}

fn vmovdqu8_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 139, 111, 18], OperandSize::Dword)
}

fn vmovdqu8_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 127, 142, 111, 224], OperandSize::Qword)
}

fn vmovdqu8_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RSI, 756772570, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 139, 111, 182, 218, 110, 27, 45], OperandSize::Qword)
}

fn vmovdqu8_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 172, 111, 240], OperandSize::Dword)
}

fn vmovdqu8_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1288004266, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 170, 111, 180, 72, 170, 98, 197, 76], OperandSize::Dword)
}

fn vmovdqu8_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 127, 172, 111, 219], OperandSize::Qword)
}

fn vmovdqu8_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM13)), operand2: Some(IndirectDisplaced(RBX, 1332318920, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 127, 170, 111, 171, 200, 146, 105, 79], OperandSize::Qword)
}

fn vmovdqu8_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 203, 111, 209], OperandSize::Dword)
}

fn vmovdqu8_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1010909766, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 204, 111, 132, 129, 70, 66, 65, 60], OperandSize::Dword)
}

fn vmovdqu8_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 127, 202, 111, 197], OperandSize::Qword)
}

fn vmovdqu8_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectDisplaced(RAX, 559064875, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 127, 204, 111, 176, 43, 167, 82, 33], OperandSize::Qword)
}

fn vmovdqu8_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 138, 111, 247], OperandSize::Dword)
}

fn vmovdqu8_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 127, 20, 184], OperandSize::Dword)
}

fn vmovdqu8_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 127, 142, 111, 247], OperandSize::Qword)
}

fn vmovdqu8_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectDisplaced(RBX, 842925213, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 127, 8, 127, 147, 157, 4, 62, 50], OperandSize::Qword)
}

fn vmovdqu8_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 172, 111, 212], OperandSize::Dword)
}

fn vmovdqu8_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 40, 127, 28, 219], OperandSize::Dword)
}

fn vmovdqu8_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 127, 174, 111, 245], OperandSize::Qword)
}

fn vmovdqu8_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 40, 127, 20, 78], OperandSize::Qword)
}

fn vmovdqu8_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 205, 111, 207], OperandSize::Dword)
}

fn vmovdqu8_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 72, 127, 58], OperandSize::Dword)
}

fn vmovdqu8_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 127, 204, 111, 210], OperandSize::Qword)
}

fn vmovdqu8_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 127, 72, 127, 31], OperandSize::Qword)
}

