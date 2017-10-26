use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovshdup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 192], OperandSize::Dword)
}

#[test]
fn vmovshdup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 2072408020, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 188, 152, 212, 111, 134, 123], OperandSize::Dword)
}

#[test]
fn vmovshdup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 208], OperandSize::Qword)
}

#[test]
fn vmovshdup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 28, 203], OperandSize::Qword)
}

#[test]
fn vmovshdup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 249], OperandSize::Dword)
}

#[test]
fn vmovshdup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 52, 88], OperandSize::Dword)
}

#[test]
fn vmovshdup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 207], OperandSize::Qword)
}

#[test]
fn vmovshdup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 22], OperandSize::Qword)
}

#[test]
fn vmovshdup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 22, 194], OperandSize::Dword)
}

#[test]
fn vmovshdup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 22, 28, 66], OperandSize::Dword)
}

#[test]
fn vmovshdup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 126, 141, 22, 216], OperandSize::Qword)
}

#[test]
fn vmovshdup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 500266160, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 126, 141, 22, 148, 94, 176, 116, 209, 29], OperandSize::Qword)
}

#[test]
fn vmovshdup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 173, 22, 242], OperandSize::Dword)
}

#[test]
fn vmovshdup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1686761323, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 173, 22, 148, 183, 107, 239, 137, 100], OperandSize::Dword)
}

#[test]
fn vmovshdup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 126, 175, 22, 220], OperandSize::Qword)
}

#[test]
fn vmovshdup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RBX, 1491932015, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 22, 179, 111, 19, 237, 88], OperandSize::Qword)
}

#[test]
fn vmovshdup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 22, 252], OperandSize::Dword)
}

#[test]
fn vmovshdup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 955989525, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 204, 22, 36, 197, 21, 62, 251, 56], OperandSize::Dword)
}

#[test]
fn vmovshdup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 126, 204, 22, 231], OperandSize::Qword)
}

#[test]
fn vmovshdup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM13)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 126, 204, 22, 47], OperandSize::Qword)
}

