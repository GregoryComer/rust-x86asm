use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpternlogq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 142, 37, 212, 91], OperandSize::Dword)
}

#[test]
fn vpternlogq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1150027217, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 221, 138, 37, 140, 144, 209, 5, 140, 68, 122], OperandSize::Dword)
}

#[test]
fn vpternlogq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1366727432, Some(OperandSize::Qword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 197, 159, 37, 164, 152, 8, 155, 118, 81, 76], OperandSize::Dword)
}

#[test]
fn vpternlogq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM25)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 131, 253, 143, 37, 249, 77], OperandSize::Qword)
}

#[test]
fn vpternlogq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1036717633, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 181, 135, 37, 44, 205, 65, 14, 203, 61, 30], OperandSize::Qword)
}

#[test]
fn vpternlogq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 2076984848, Some(OperandSize::Qword), None)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 157, 147, 37, 132, 246, 16, 70, 204, 123, 50], OperandSize::Qword)
}

#[test]
fn vpternlogq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 197, 173, 37, 247, 88], OperandSize::Dword)
}

#[test]
fn vpternlogq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 820498244, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 229, 172, 37, 36, 253, 68, 207, 231, 48, 37], OperandSize::Dword)
}

#[test]
fn vpternlogq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 245, 189, 37, 28, 66, 52], OperandSize::Dword)
}

#[test]
fn vpternlogq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM20)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 35, 157, 162, 37, 228, 124], OperandSize::Qword)
}

#[test]
fn vpternlogq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 719779310, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 221, 161, 37, 36, 141, 238, 245, 230, 42, 97], OperandSize::Qword)
}

#[test]
fn vpternlogq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 410403112, Some(OperandSize::Qword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 141, 181, 37, 188, 176, 40, 65, 118, 24, 39], OperandSize::Qword)
}

#[test]
fn vpternlogq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 202, 37, 222, 17], OperandSize::Dword)
}

#[test]
fn vpternlogq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 245, 204, 37, 36, 241, 126], OperandSize::Dword)
}

#[test]
fn vpternlogq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 205, 218, 37, 32, 108], OperandSize::Dword)
}

#[test]
fn vpternlogq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM11)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 189, 199, 37, 211, 20], OperandSize::Qword)
}

#[test]
fn vpternlogq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 543214567, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 221, 204, 37, 52, 125, 231, 203, 96, 32, 31], OperandSize::Qword)
}

#[test]
fn vpternlogq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectDisplaced(RAX, 952645621, Some(OperandSize::Qword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 173, 223, 37, 152, 245, 55, 200, 56, 101], OperandSize::Qword)
}

