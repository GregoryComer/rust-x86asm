use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 140, 102, 200], OperandSize::Dword)
}

#[test]
fn vpblendmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 137, 102, 44, 183], OperandSize::Dword)
}

#[test]
fn vpblendmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 197, 138, 102, 251], OperandSize::Qword)
}

#[test]
fn vpblendmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 275246763, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 157, 137, 102, 28, 253, 171, 238, 103, 16], OperandSize::Qword)
}

#[test]
fn vpblendmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 173, 102, 236], OperandSize::Dword)
}

#[test]
fn vpblendmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 875490732, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 172, 102, 167, 172, 237, 46, 52], OperandSize::Dword)
}

#[test]
fn vpblendmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 253, 164, 102, 227], OperandSize::Qword)
}

#[test]
fn vpblendmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 213, 173, 102, 9], OperandSize::Qword)
}

#[test]
fn vpblendmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 205, 102, 235], OperandSize::Dword)
}

#[test]
fn vpblendmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 957835387, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 102, 140, 185, 123, 104, 23, 57], OperandSize::Dword)
}

#[test]
fn vpblendmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 165, 199, 102, 240], OperandSize::Qword)
}

#[test]
fn vpblendmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 203, 102, 33], OperandSize::Qword)
}

