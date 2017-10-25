use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 208], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 24], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 244], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 34], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 239], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 3], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 197], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 463007927, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 12, 253, 183, 240, 152, 27], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 32, 199], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 25133624, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 32, 161, 56, 130, 127, 1], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 125, 140, 32, 198], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 327615037, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 137, 32, 140, 218, 61, 2, 135, 19], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 32, 218], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 32, 54], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 125, 172, 32, 238], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM26)), operand2: Some(IndirectDisplaced(RCX, 880923861, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 170, 32, 145, 213, 212, 129, 52], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 32, 238], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 32, 4, 88], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 32, 215], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 32, 63], OperandSize::Qword)
}

