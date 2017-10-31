use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 139, 118, 206], OperandSize::Dword)
}

#[test]
fn vpermi2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1894856872, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 142, 118, 36, 253, 168, 56, 241, 112], OperandSize::Dword)
}

#[test]
fn vpermi2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1817593315, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 155, 118, 172, 112, 227, 69, 86, 108], OperandSize::Dword)
}

#[test]
fn vpermi2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 157, 129, 118, 255], OperandSize::Qword)
}

#[test]
fn vpermi2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1678958328, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 173, 129, 118, 20, 213, 248, 222, 18, 100], OperandSize::Qword)
}

#[test]
fn vpermi2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 85440096, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 213, 158, 118, 60, 93, 96, 182, 23, 5], OperandSize::Qword)
}

#[test]
fn vpermi2q_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 118, 241], OperandSize::Dword)
}

#[test]
fn vpermi2q_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 118, 0], OperandSize::Dword)
}

#[test]
fn vpermi2q_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 189, 118, 20, 218], OperandSize::Dword)
}

#[test]
fn vpermi2q_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 197, 174, 118, 194], OperandSize::Qword)
}

#[test]
fn vpermi2q_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 34975880, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 205, 172, 118, 36, 117, 136, 176, 21, 2], OperandSize::Qword)
}

#[test]
fn vpermi2q_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1065516154, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 133, 188, 118, 28, 181, 122, 124, 130, 63], OperandSize::Qword)
}

#[test]
fn vpermi2q_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 203, 118, 246], OperandSize::Dword)
}

#[test]
fn vpermi2q_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 206, 118, 36, 114], OperandSize::Dword)
}

#[test]
fn vpermi2q_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 794007491, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 219, 118, 52, 85, 195, 151, 83, 47], OperandSize::Dword)
}

#[test]
fn vpermi2q_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 149, 193, 118, 200], OperandSize::Qword)
}

#[test]
fn vpermi2q_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 116002, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 157, 194, 118, 188, 193, 34, 197, 1, 0], OperandSize::Qword)
}

#[test]
fn vpermi2q_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 393505752, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 133, 219, 118, 148, 242, 216, 107, 116, 23], OperandSize::Qword)
}

