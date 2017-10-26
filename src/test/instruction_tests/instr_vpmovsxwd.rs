use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 250], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 15], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 247], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 36, 114], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 253], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 950436157, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 156, 146, 61, 129, 166, 56], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 227], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(RDI, 1632204033, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 183, 1, 117, 73, 97], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 35, 253], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 2097757591, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 35, 12, 197, 151, 61, 9, 125], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 35, 209], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM24)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 141, 35, 1], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 35, 231], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1884605078, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 35, 44, 149, 150, 202, 84, 112], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 125, 171, 35, 236], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM26)), operand2: Some(IndirectDisplaced(RSI, 2133826894, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 174, 35, 150, 78, 157, 47, 127], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 35, 246], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 35, 26], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 125, 201, 35, 238], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM13)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 204, 35, 47], OperandSize::Qword)
}

