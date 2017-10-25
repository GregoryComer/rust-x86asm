use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 21, 207], OperandSize::Dword)
}

#[test]
fn vprolvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 21071088, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 143, 21, 134, 240, 132, 65, 1], OperandSize::Dword)
}

#[test]
fn vprolvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 2022351744, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 158, 21, 36, 221, 128, 163, 138, 120], OperandSize::Dword)
}

#[test]
fn vprolvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 253, 137, 21, 237], OperandSize::Qword)
}

#[test]
fn vprolvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1646694593, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 173, 139, 21, 180, 74, 193, 144, 38, 98], OperandSize::Qword)
}

#[test]
fn vprolvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 327738169, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 189, 145, 21, 140, 219, 57, 227, 136, 19], OperandSize::Qword)
}

#[test]
fn vprolvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 169, 21, 204], OperandSize::Dword)
}

#[test]
fn vprolvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1719549217, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 169, 21, 44, 125, 33, 61, 126, 102], OperandSize::Dword)
}

#[test]
fn vprolvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 950323657, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 186, 21, 158, 201, 201, 164, 56], OperandSize::Dword)
}

#[test]
fn vprolvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 165, 166, 21, 224], OperandSize::Qword)
}

#[test]
fn vprolvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM14)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 141, 169, 21, 41], OperandSize::Qword)
}

#[test]
fn vprolvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 2125120125, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 221, 188, 21, 20, 117, 125, 194, 170, 126], OperandSize::Qword)
}

#[test]
fn vprolvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 202, 21, 199], OperandSize::Dword)
}

#[test]
fn vprolvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 1718436861, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 205, 21, 148, 185, 253, 67, 109, 102], OperandSize::Dword)
}

#[test]
fn vprolvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 1499420078, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 222, 21, 148, 89, 174, 85, 95, 89], OperandSize::Dword)
}

#[test]
fn vprolvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 181, 198, 21, 218], OperandSize::Qword)
}

#[test]
fn vprolvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1011396280, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 253, 197, 21, 12, 189, 184, 174, 72, 60], OperandSize::Qword)
}

#[test]
fn vprolvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 379736863, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 205, 222, 21, 180, 206, 31, 83, 162, 22], OperandSize::Qword)
}

