use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 151, 222], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 298349116, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 151, 180, 246, 60, 114, 200, 17], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 151, 207], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 838368707, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 151, 28, 141, 195, 125, 248, 49], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 151, 240], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 151, 63], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 151, 195], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDI, 1812982396, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 151, 151, 124, 234, 15, 108], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 139, 151, 228], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 1745043226, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 141, 151, 134, 26, 63, 3, 104], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 2101821685, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 159, 151, 60, 77, 245, 64, 71, 125], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 213, 129, 151, 238], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 229, 141, 151, 28, 217], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RBX, 1381512612, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 133, 159, 151, 155, 164, 53, 88, 82], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 151, 206], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 924657212, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 151, 172, 254, 60, 38, 29, 55], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 185, 151, 28, 184], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 229, 169, 151, 231], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectDisplaced(RDX, 1928142269, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 229, 165, 151, 178, 189, 29, 237, 114], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 501510015, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 173, 191, 151, 60, 221, 127, 111, 228, 29], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 223, 151, 225], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 919272959, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 151, 164, 64, 255, 253, 202, 54], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EAX, 334108423, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 220, 151, 168, 7, 23, 234, 19], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 253, 218, 151, 214], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 213, 201, 151, 44, 251], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1865960394, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 211, 151, 20, 253, 202, 75, 56, 111], OperandSize::Qword)
}

