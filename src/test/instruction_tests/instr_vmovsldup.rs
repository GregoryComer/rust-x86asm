use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovsldup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 247], OperandSize::Dword)
}

#[test]
fn vmovsldup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 38], OperandSize::Dword)
}

#[test]
fn vmovsldup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 233], OperandSize::Qword)
}

#[test]
fn vmovsldup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RAX, 117323389, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 18, 136, 125, 54, 254, 6], OperandSize::Qword)
}

#[test]
fn vmovsldup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 234], OperandSize::Dword)
}

#[test]
fn vmovsldup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 824435550, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 36, 189, 94, 227, 35, 49], OperandSize::Dword)
}

#[test]
fn vmovsldup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 220], OperandSize::Qword)
}

#[test]
fn vmovsldup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 18, 25], OperandSize::Qword)
}

#[test]
fn vmovsldup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 18, 249], OperandSize::Dword)
}

#[test]
fn vmovsldup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1030787198, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 18, 172, 67, 126, 144, 112, 61], OperandSize::Dword)
}

#[test]
fn vmovsldup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 126, 139, 18, 236], OperandSize::Qword)
}

#[test]
fn vmovsldup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(XMM25)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 126, 143, 18, 15], OperandSize::Qword)
}

#[test]
fn vmovsldup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 18, 207], OperandSize::Dword)
}

#[test]
fn vmovsldup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 2030802927, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 172, 18, 164, 207, 239, 151, 11, 121], OperandSize::Dword)
}

#[test]
fn vmovsldup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 126, 173, 18, 226], OperandSize::Qword)
}

#[test]
fn vmovsldup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 308254412, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 126, 172, 18, 60, 117, 204, 150, 95, 18], OperandSize::Qword)
}

#[test]
fn vmovsldup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 18, 245], OperandSize::Dword)
}

#[test]
fn vmovsldup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 1798825499, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 18, 156, 120, 27, 230, 55, 107], OperandSize::Dword)
}

#[test]
fn vmovsldup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 126, 205, 18, 222], OperandSize::Qword)
}

#[test]
fn vmovsldup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSLDUP, operand1: Some(Direct(ZMM18)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 126, 204, 18, 17], OperandSize::Qword)
}

