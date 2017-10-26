use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 234, 213], OperandSize::Dword)
}

#[test]
fn vpminsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ECX, 1998181604, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 234, 185, 228, 212, 25, 119], OperandSize::Dword)
}

#[test]
fn vpminsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 234, 192], OperandSize::Qword)
}

#[test]
fn vpminsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 234, 35], OperandSize::Qword)
}

#[test]
fn vpminsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 234, 239], OperandSize::Dword)
}

#[test]
fn vpminsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1635933617, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 234, 44, 117, 177, 93, 130, 97], OperandSize::Dword)
}

#[test]
fn vpminsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 234, 245], OperandSize::Qword)
}

#[test]
fn vpminsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 234, 12, 200], OperandSize::Qword)
}

#[test]
fn vpminsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 234, 205], OperandSize::Dword)
}

#[test]
fn vpminsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 2107852272, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 234, 12, 197, 240, 69, 163, 125], OperandSize::Dword)
}

#[test]
fn vpminsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 117, 140, 234, 213], OperandSize::Qword)
}

#[test]
fn vpminsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 13, 140, 234, 60, 146], OperandSize::Qword)
}

#[test]
fn vpminsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 173, 234, 243], OperandSize::Dword)
}

#[test]
fn vpminsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 234, 36, 192], OperandSize::Dword)
}

#[test]
fn vpminsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 117, 171, 234, 200], OperandSize::Qword)
}

#[test]
fn vpminsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDI, 1661581210, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 125, 174, 234, 183, 154, 183, 9, 99], OperandSize::Qword)
}

#[test]
fn vpminsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 234, 223], OperandSize::Dword)
}

#[test]
fn vpminsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1266471181, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 205, 234, 12, 93, 13, 209, 124, 75], OperandSize::Dword)
}

#[test]
fn vpminsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 125, 199, 234, 230], OperandSize::Qword)
}

#[test]
fn vpminsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RAX, 1861322570, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 5, 207, 234, 128, 74, 135, 241, 110], OperandSize::Qword)
}

