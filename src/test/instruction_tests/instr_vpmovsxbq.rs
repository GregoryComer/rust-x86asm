use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 214], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 36, 195], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 210], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 34, 36, 186], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 246], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 12, 243], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 196], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 34, 34], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 34, 200], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 34, 40], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 125, 137, 34, 253], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RBX, 1984922848, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 34, 139, 224, 132, 79, 118], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 34, 232], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1647109916, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 34, 36, 77, 28, 231, 44, 98], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 125, 175, 34, 231], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(YMM22)), operand2: Some(IndirectDisplaced(RSI, 1512600525, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 173, 34, 182, 205, 115, 40, 90], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 34, 238], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 921141833, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 34, 60, 77, 73, 130, 231, 54], OperandSize::Dword)
}

#[test]
fn vpmovsxbq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 205, 34, 237], OperandSize::Qword)
}

#[test]
fn vpmovsxbq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBQ, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1343891413, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 207, 34, 180, 70, 213, 39, 26, 80], OperandSize::Qword)
}

