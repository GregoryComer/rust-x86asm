use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovdqa32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 111, 202], OperandSize::Dword)
}

fn vmovdqa32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 1672345353, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 111, 156, 198, 9, 247, 173, 99], OperandSize::Dword)
}

fn vmovdqa32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 125, 138, 111, 203], OperandSize::Qword)
}

fn vmovdqa32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 125, 141, 111, 52, 193], OperandSize::Qword)
}

fn vmovdqa32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 111, 236], OperandSize::Dword)
}

fn vmovdqa32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 111, 55], OperandSize::Dword)
}

fn vmovdqa32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 111, 234], OperandSize::Qword)
}

fn vmovdqa32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM29)), operand2: Some(IndirectDisplaced(RAX, 873558309, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 125, 172, 111, 168, 37, 113, 17, 52], OperandSize::Qword)
}

fn vmovdqa32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 111, 222], OperandSize::Dword)
}

fn vmovdqa32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(EBX, 1760273984, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 111, 147, 64, 166, 235, 104], OperandSize::Dword)
}

fn vmovdqa32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 125, 206, 111, 241], OperandSize::Qword)
}

fn vmovdqa32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 949836057, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 125, 201, 111, 12, 245, 25, 89, 157, 56], OperandSize::Qword)
}

fn vmovdqa32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 111, 210], OperandSize::Dword)
}

fn vmovdqa32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 8, 127, 22], OperandSize::Dword)
}

fn vmovdqa32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 125, 137, 111, 241], OperandSize::Qword)
}

fn vmovdqa32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 125, 8, 127, 43], OperandSize::Qword)
}

fn vmovdqa32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 111, 199], OperandSize::Dword)
}

fn vmovdqa32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 689593928, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 40, 127, 132, 144, 72, 94, 26, 41], OperandSize::Dword)
}

fn vmovdqa32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 125, 171, 111, 209], OperandSize::Qword)
}

fn vmovdqa32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 2029221152, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 125, 40, 127, 188, 94, 32, 117, 243, 120], OperandSize::Qword)
}

fn vmovdqa32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 111, 204], OperandSize::Dword)
}

fn vmovdqa32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 127, 7], OperandSize::Dword)
}

fn vmovdqa32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 125, 206, 111, 234], OperandSize::Qword)
}

fn vmovdqa32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1222739027, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 125, 72, 127, 4, 181, 83, 132, 225, 72], OperandSize::Qword)
}

