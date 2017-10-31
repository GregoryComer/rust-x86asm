use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovshdup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 195], OperandSize::Dword)
}

#[test]
fn vmovshdup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 62], OperandSize::Dword)
}

#[test]
fn vmovshdup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 203], OperandSize::Qword)
}

#[test]
fn vmovshdup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 55276533, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 172, 177, 245, 115, 75, 3], OperandSize::Qword)
}

#[test]
fn vmovshdup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 251], OperandSize::Dword)
}

#[test]
fn vmovshdup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EDI, 1957743201, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 183, 97, 202, 176, 116], OperandSize::Dword)
}

#[test]
fn vmovshdup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 232], OperandSize::Qword)
}

#[test]
fn vmovshdup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(RBX, 605405330, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 187, 146, 192, 21, 36], OperandSize::Qword)
}

#[test]
fn vmovshdup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 22, 232], OperandSize::Dword)
}

#[test]
fn vmovshdup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 997345665, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 22, 156, 207, 129, 73, 114, 59], OperandSize::Dword)
}

#[test]
fn vmovshdup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 126, 137, 22, 220], OperandSize::Qword)
}

#[test]
fn vmovshdup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM20)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 126, 137, 22, 35], OperandSize::Qword)
}

#[test]
fn vmovshdup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 22, 235], OperandSize::Dword)
}

#[test]
fn vmovshdup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1224412178, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 173, 22, 44, 253, 18, 12, 251, 72], OperandSize::Dword)
}

#[test]
fn vmovshdup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 126, 175, 22, 213], OperandSize::Qword)
}

#[test]
fn vmovshdup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 126, 169, 22, 20, 246], OperandSize::Qword)
}

#[test]
fn vmovshdup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 22, 203], OperandSize::Dword)
}

#[test]
fn vmovshdup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1769775239, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 201, 22, 132, 159, 135, 160, 124, 105], OperandSize::Dword)
}

#[test]
fn vmovshdup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 126, 201, 22, 213], OperandSize::Qword)
}

#[test]
fn vmovshdup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM22)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 126, 203, 22, 52, 135], OperandSize::Qword)
}

