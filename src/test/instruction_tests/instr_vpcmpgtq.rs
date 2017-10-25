use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 55, 217], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 159672209, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 55, 188, 185, 145, 103, 132, 9], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 55, 215], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1881511138, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 55, 132, 71, 226, 148, 37, 112], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 55, 208], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 55, 17], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 55, 218], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 55, 44, 218], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 15, 55, 251], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 11, 55, 35], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 2083835696, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 26, 55, 180, 95, 48, 207, 52, 124], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 141, 15, 55, 225], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 162778643, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 15, 55, 28, 205, 19, 206, 179, 9], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 984041007, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 149, 21, 55, 28, 85, 47, 70, 167, 58], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 44, 55, 230], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 45, 55, 27], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 609160802, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 59, 55, 36, 253, 98, 14, 79, 36], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 181, 35, 55, 229], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 2144987047, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 46, 55, 172, 86, 167, 231, 217, 127], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1588651052, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 60, 55, 44, 189, 44, 228, 176, 94], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 74, 55, 225], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 1963962961, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 74, 55, 140, 121, 81, 178, 15, 117], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1815925767, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 93, 55, 28, 181, 7, 212, 60, 108], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 141, 68, 55, 248], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 189, 74, 55, 52, 194], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 86, 55, 12, 211], OperandSize::Qword)
}

