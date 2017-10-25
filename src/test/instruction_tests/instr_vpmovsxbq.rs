use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 205], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 36, 65], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 223], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1828473187, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 4, 205, 99, 73, 252, 108], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 220], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 43], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 236], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 20, 83], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 34, 249], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 197627364, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 34, 60, 77, 228, 141, 199, 11], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 142, 34, 244], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM12)), operand2: Some(IndirectDisplaced(RSI, 1312753501, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 138, 34, 166, 93, 7, 63, 78], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 34, 251], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 34, 43], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM25)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 125, 175, 34, 200], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 169, 34, 36, 95], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 34, 249], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 34, 20, 83], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 125, 201, 34, 200], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 34, 36, 154], OperandSize::Qword)
}

