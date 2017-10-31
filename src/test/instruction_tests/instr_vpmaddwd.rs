use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaddwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 245, 197], OperandSize::Dword)
}

#[test]
fn vpmaddwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1531290657, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 245, 131, 33, 164, 69, 91], OperandSize::Dword)
}

#[test]
fn vpmaddwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 245, 209], OperandSize::Qword)
}

#[test]
fn vpmaddwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 2082457777, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 245, 60, 205, 177, 200, 31, 124], OperandSize::Qword)
}

#[test]
fn vpmaddwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 245, 199], OperandSize::Dword)
}

#[test]
fn vpmaddwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 1339983814, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 245, 172, 247, 198, 135, 222, 79], OperandSize::Dword)
}

#[test]
fn vpmaddwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 245, 194], OperandSize::Qword)
}

#[test]
fn vpmaddwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RDI, 1477852406, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 245, 159, 246, 60, 22, 88], OperandSize::Qword)
}

#[test]
fn vpmaddwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 245, 201], OperandSize::Dword)
}

#[test]
fn vpmaddwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1076715772, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 137, 245, 188, 155, 252, 96, 45, 64], OperandSize::Dword)
}

#[test]
fn vpmaddwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 45, 139, 245, 240], OperandSize::Qword)
}

#[test]
fn vpmaddwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1270715154, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 37, 137, 245, 172, 126, 18, 147, 189, 75], OperandSize::Qword)
}

#[test]
fn vpmaddwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 175, 245, 213], OperandSize::Dword)
}

#[test]
fn vpmaddwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 2118410889, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 169, 245, 156, 86, 137, 98, 68, 126], OperandSize::Dword)
}

#[test]
fn vpmaddwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 5, 164, 245, 244], OperandSize::Qword)
}

#[test]
fn vpmaddwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 37, 162, 245, 36, 115], OperandSize::Qword)
}

#[test]
fn vpmaddwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 245, 237], OperandSize::Dword)
}

#[test]
fn vpmaddwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 245, 44, 65], OperandSize::Dword)
}

#[test]
fn vpmaddwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 29, 205, 245, 232], OperandSize::Qword)
}

#[test]
fn vpmaddwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM21)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 85, 199, 245, 16], OperandSize::Qword)
}

