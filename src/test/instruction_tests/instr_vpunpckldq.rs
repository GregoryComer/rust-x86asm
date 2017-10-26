use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 98, 230], OperandSize::Dword)
}

#[test]
fn vpunpckldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 98, 35], OperandSize::Dword)
}

#[test]
fn vpunpckldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 98, 225], OperandSize::Qword)
}

#[test]
fn vpunpckldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1410903513, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 98, 36, 197, 217, 173, 24, 84], OperandSize::Qword)
}

#[test]
fn vpunpckldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 98, 197], OperandSize::Dword)
}

#[test]
fn vpunpckldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDX, 2079612120, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 98, 130, 216, 92, 244, 123], OperandSize::Dword)
}

#[test]
fn vpunpckldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 98, 195], OperandSize::Qword)
}

#[test]
fn vpunpckldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1634753021, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 98, 156, 120, 253, 89, 112, 97], OperandSize::Qword)
}

#[test]
fn vpunpckldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 98, 238], OperandSize::Dword)
}

#[test]
fn vpunpckldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 142, 98, 36, 155], OperandSize::Dword)
}

#[test]
fn vpunpckldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 1634139308, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 159, 98, 139, 172, 252, 102, 97], OperandSize::Dword)
}

#[test]
fn vpunpckldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 93, 141, 98, 233], OperandSize::Qword)
}

#[test]
fn vpunpckldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 135, 98, 26], OperandSize::Qword)
}

#[test]
fn vpunpckldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 5, 145, 98, 4, 243], OperandSize::Qword)
}

