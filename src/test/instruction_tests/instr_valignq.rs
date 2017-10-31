use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn valignq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 245, 137, 3, 204, 39], OperandSize::Dword)
}

#[test]
fn valignq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 245, 138, 3, 20, 178, 111], OperandSize::Dword)
}

#[test]
fn valignq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 229, 156, 3, 36, 66, 96], OperandSize::Dword)
}

#[test]
fn valignq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM22)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 35, 181, 141, 3, 222, 49], OperandSize::Qword)
}

#[test]
fn valignq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 237, 132, 3, 26, 119], OperandSize::Qword)
}

#[test]
fn valignq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 213, 153, 3, 10, 94], OperandSize::Qword)
}

#[test]
fn valignq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 169, 3, 224, 63], OperandSize::Dword)
}

#[test]
fn valignq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 363272726, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 173, 3, 172, 119, 22, 26, 167, 21, 36], OperandSize::Dword)
}

#[test]
fn valignq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1285021535, Some(OperandSize::Qword), None)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 197, 188, 3, 44, 77, 95, 223, 151, 76, 31], OperandSize::Dword)
}

#[test]
fn valignq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM20)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 197, 173, 3, 252, 116], OperandSize::Qword)
}

#[test]
fn valignq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 133, 173, 3, 20, 95, 30], OperandSize::Qword)
}

#[test]
fn valignq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 1897154538, Some(OperandSize::Qword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 141, 187, 3, 172, 243, 234, 71, 20, 113, 97], OperandSize::Qword)
}

#[test]
fn valignq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 197, 201, 3, 210, 29], OperandSize::Dword)
}

#[test]
fn valignq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EBX, 1967284060, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 229, 203, 3, 139, 92, 95, 66, 117, 48], OperandSize::Dword)
}

#[test]
fn valignq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 237, 221, 3, 60, 81, 100], OperandSize::Dword)
}

#[test]
fn valignq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM22)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 35, 245, 195, 3, 254, 119], OperandSize::Qword)
}

#[test]
fn valignq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RBX, 1241123465, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 229, 198, 3, 131, 137, 10, 250, 73, 9], OperandSize::Qword)
}

#[test]
fn valignq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 237, 212, 3, 51, 44], OperandSize::Qword)
}

