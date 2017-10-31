use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 41, 218], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1464127729, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 41, 28, 85, 241, 208, 68, 87], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 41, 205], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 41, 46], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 41, 220], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 41, 44, 126], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 41, 235], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 218622586, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 41, 180, 178, 122, 234, 7, 13], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 12, 41, 244], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1079036316, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 12, 41, 44, 205, 156, 201, 80, 64], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 30, 41, 44, 222], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 181, 11, 41, 224], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 2125345296, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 7, 41, 44, 77, 16, 50, 174, 126], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 21, 41, 20, 67], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 45, 41, 206], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 43, 41, 43], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1522451199, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 57, 41, 52, 181, 255, 194, 190, 90], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 173, 38, 41, 239], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 39, 41, 60, 223], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 2005498247, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 54, 41, 156, 223, 135, 121, 137, 119], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 76, 41, 235], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1152696961, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 75, 41, 148, 139, 129, 194, 180, 68], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 784811833, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 94, 41, 164, 120, 57, 71, 199, 46], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 157, 69, 41, 212], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 78, 41, 12, 134], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 173, 86, 41, 20, 223], OperandSize::Qword)
}

