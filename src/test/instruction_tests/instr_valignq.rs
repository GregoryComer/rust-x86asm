use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn valignq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 205, 138, 3, 234, 125], OperandSize::Dword)
}

#[test]
fn valignq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 1390824013, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 237, 143, 3, 142, 77, 74, 230, 82, 86], OperandSize::Dword)
}

#[test]
fn valignq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 245, 153, 3, 23, 21], OperandSize::Dword)
}

#[test]
fn valignq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM18)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 51, 245, 133, 3, 210, 81], OperandSize::Qword)
}

#[test]
fn valignq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 902140643, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 253, 131, 3, 164, 251, 227, 146, 197, 53, 120], OperandSize::Qword)
}

#[test]
fn valignq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 173, 150, 3, 24, 36], OperandSize::Qword)
}

#[test]
fn valignq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 221, 169, 3, 223, 90], OperandSize::Dword)
}

#[test]
fn valignq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 229, 169, 3, 34, 40], OperandSize::Dword)
}

#[test]
fn valignq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 1898047022, Some(OperandSize::Qword), None)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 245, 191, 3, 145, 46, 230, 33, 113, 74], OperandSize::Dword)
}

#[test]
fn valignq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM26)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 3, 205, 173, 3, 226, 39], OperandSize::Qword)
}

#[test]
fn valignq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 205, 166, 3, 60, 155, 13], OperandSize::Qword)
}

#[test]
fn valignq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 173, 180, 3, 38, 21], OperandSize::Qword)
}

#[test]
fn valignq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 229, 207, 3, 209, 43], OperandSize::Dword)
}

#[test]
fn valignq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 213, 207, 3, 15, 110], OperandSize::Dword)
}

#[test]
fn valignq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 245, 219, 3, 41, 49], OperandSize::Dword)
}

#[test]
fn valignq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM21)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 35, 237, 207, 3, 197, 60], OperandSize::Qword)
}

#[test]
fn valignq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 587964226, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 181, 197, 3, 36, 253, 66, 159, 11, 35, 56], OperandSize::Qword)
}

#[test]
fn valignq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM11)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 165, 221, 3, 25, 59], OperandSize::Qword)
}

