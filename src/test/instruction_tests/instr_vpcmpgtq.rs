use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 55, 243], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 55, 12, 75], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 55, 228], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 398438930, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 55, 164, 75, 18, 178, 191, 23], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 55, 210], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1257522399, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 55, 156, 208, 223, 68, 244, 74], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 55, 235], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 37513289, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 55, 60, 117, 73, 104, 60, 2], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 13, 55, 252], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 9, 55, 62], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 480899201, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 28, 55, 60, 245, 129, 240, 169, 28], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 157, 13, 55, 216], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 12, 55, 28, 146], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 21, 55, 38], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 41, 55, 217], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 46, 55, 19], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 851960777, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 60, 55, 171, 201, 227, 199, 50], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 149, 44, 55, 230], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 42, 55, 63], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 164158672, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 54, 55, 36, 181, 208, 220, 200, 9], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 73, 55, 225], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 78, 55, 9], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 91, 55, 44, 202], OperandSize::Dword)
}

#[test]
fn vpcmpgtq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 181, 74, 55, 249], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 71, 55, 44, 83], OperandSize::Qword)
}

#[test]
fn vpcmpgtq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectDisplaced(RBX, 1622878914, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 83, 55, 155, 194, 42, 187, 96], OperandSize::Qword)
}

