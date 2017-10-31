use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 69, 235], OperandSize::Dword)
}

#[test]
fn vpsrlvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 69, 1], OperandSize::Dword)
}

#[test]
fn vpsrlvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 69, 243], OperandSize::Qword)
}

#[test]
fn vpsrlvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1720587772, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 69, 12, 253, 252, 21, 142, 102], OperandSize::Qword)
}

#[test]
fn vpsrlvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 69, 233], OperandSize::Dword)
}

#[test]
fn vpsrlvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 69, 44, 216], OperandSize::Dword)
}

#[test]
fn vpsrlvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 69, 241], OperandSize::Qword)
}

#[test]
fn vpsrlvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDX, 1536702913, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 69, 130, 193, 57, 152, 91], OperandSize::Qword)
}

#[test]
fn vpsrlvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 69, 237], OperandSize::Dword)
}

#[test]
fn vpsrlvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1915632976, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 69, 132, 254, 80, 61, 46, 114], OperandSize::Dword)
}

#[test]
fn vpsrlvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 159, 69, 4, 88], OperandSize::Dword)
}

#[test]
fn vpsrlvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 149, 138, 69, 229], OperandSize::Qword)
}

#[test]
fn vpsrlvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1822562333, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 213, 130, 69, 12, 253, 29, 24, 162, 108], OperandSize::Qword)
}

#[test]
fn vpsrlvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1424111227, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 205, 146, 69, 28, 221, 123, 54, 226, 84], OperandSize::Qword)
}

#[test]
fn vpsrlvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 171, 69, 240], OperandSize::Dword)
}

#[test]
fn vpsrlvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1062293552, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 69, 4, 253, 48, 80, 81, 63], OperandSize::Dword)
}

#[test]
fn vpsrlvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 1300405861, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 189, 69, 154, 101, 158, 130, 77], OperandSize::Dword)
}

#[test]
fn vpsrlvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 181, 172, 69, 198], OperandSize::Qword)
}

#[test]
fn vpsrlvq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1513866230, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 189, 171, 69, 188, 194, 246, 195, 59, 90], OperandSize::Qword)
}

#[test]
fn vpsrlvq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1578758396, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 181, 182, 69, 52, 117, 252, 240, 25, 94], OperandSize::Qword)
}

#[test]
fn vpsrlvq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 203, 69, 229], OperandSize::Dword)
}

#[test]
fn vpsrlvq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 928188881, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 203, 69, 140, 210, 209, 9, 83, 55], OperandSize::Dword)
}

#[test]
fn vpsrlvq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 223, 69, 0], OperandSize::Dword)
}

#[test]
fn vpsrlvq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 149, 194, 69, 233], OperandSize::Qword)
}

#[test]
fn vpsrlvq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1383677849, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 141, 205, 69, 36, 69, 153, 63, 121, 82], OperandSize::Qword)
}

#[test]
fn vpsrlvq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 914026449, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 221, 217, 69, 60, 77, 209, 239, 122, 54], OperandSize::Qword)
}

