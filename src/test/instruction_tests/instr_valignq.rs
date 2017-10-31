use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn valignq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 138, 3, 201, 119], OperandSize::Dword)
}

#[test]
fn valignq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 927405905, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 205, 138, 3, 170, 81, 23, 71, 55, 106], OperandSize::Dword)
}

#[test]
fn valignq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 289604565, Some(OperandSize::Qword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 213, 156, 3, 180, 208, 213, 3, 67, 17, 81], OperandSize::Dword)
}

#[test]
fn valignq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM24)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 3, 149, 129, 3, 192, 59], OperandSize::Qword)
}

#[test]
fn valignq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 304950370, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 221, 142, 3, 36, 213, 98, 44, 45, 18, 16], OperandSize::Qword)
}

#[test]
fn valignq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 213, 150, 3, 52, 247, 65], OperandSize::Qword)
}

#[test]
fn valignq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 173, 3, 212, 5], OperandSize::Dword)
}

#[test]
fn valignq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 229, 169, 3, 36, 86, 61], OperandSize::Dword)
}

#[test]
fn valignq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 524596591, Some(OperandSize::Qword), None)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 245, 187, 3, 188, 223, 111, 181, 68, 31, 19], OperandSize::Dword)
}

#[test]
fn valignq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 173, 163, 3, 210, 63], OperandSize::Qword)
}

#[test]
fn valignq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RAX, 829263425, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 245, 173, 3, 136, 65, 142, 109, 49, 126], OperandSize::Qword)
}

#[test]
fn valignq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RDX, 1258992283, Some(OperandSize::Qword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 189, 181, 3, 154, 155, 178, 10, 75, 98], OperandSize::Qword)
}

#[test]
fn valignq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 205, 207, 3, 224, 41], OperandSize::Dword)
}

#[test]
fn valignq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 2048202992, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 237, 203, 3, 148, 192, 240, 24, 21, 122, 32], OperandSize::Dword)
}

#[test]
fn valignq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1958341381, Some(OperandSize::Qword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 213, 221, 3, 188, 215, 5, 235, 185, 116, 52], OperandSize::Dword)
}

#[test]
fn valignq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM29)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 19, 229, 207, 3, 253, 18], OperandSize::Qword)
}

#[test]
fn valignq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 245, 197, 3, 12, 70, 91], OperandSize::Qword)
}

#[test]
fn valignq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM24)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 189, 214, 3, 9, 82], OperandSize::Qword)
}

