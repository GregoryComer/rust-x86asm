use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovshdup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 228], OperandSize::Dword)
}

#[test]
fn vmovshdup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 629030451, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 60, 149, 51, 62, 126, 37], OperandSize::Dword)
}

#[test]
fn vmovshdup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 254], OperandSize::Qword)
}

#[test]
fn vmovshdup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 28, 177], OperandSize::Qword)
}

#[test]
fn vmovshdup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 234], OperandSize::Dword)
}

#[test]
fn vmovshdup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 69793170, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 20, 93, 146, 245, 40, 4], OperandSize::Dword)
}

#[test]
fn vmovshdup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 241], OperandSize::Qword)
}

#[test]
fn vmovshdup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 112059720, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 148, 223, 72, 229, 173, 6], OperandSize::Qword)
}

#[test]
fn vmovshdup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 22, 245], OperandSize::Dword)
}

#[test]
fn vmovshdup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDI, 1704884139, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 22, 191, 171, 119, 158, 101], OperandSize::Dword)
}

#[test]
fn vmovshdup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 126, 142, 22, 216], OperandSize::Qword)
}

#[test]
fn vmovshdup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM10)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 126, 142, 22, 16], OperandSize::Qword)
}

#[test]
fn vmovshdup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 22, 197], OperandSize::Dword)
}

#[test]
fn vmovshdup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 442139338, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 22, 28, 245, 202, 130, 90, 26], OperandSize::Dword)
}

#[test]
fn vmovshdup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 126, 169, 22, 208], OperandSize::Qword)
}

#[test]
fn vmovshdup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM10)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 701743286, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 126, 170, 22, 20, 157, 182, 192, 211, 41], OperandSize::Qword)
}

#[test]
fn vmovshdup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 22, 204], OperandSize::Dword)
}

#[test]
fn vmovshdup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 1894471878, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 22, 164, 89, 198, 88, 235, 112], OperandSize::Dword)
}

#[test]
fn vmovshdup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 126, 206, 22, 196], OperandSize::Qword)
}

#[test]
fn vmovshdup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM26)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 126, 207, 22, 22], OperandSize::Qword)
}

