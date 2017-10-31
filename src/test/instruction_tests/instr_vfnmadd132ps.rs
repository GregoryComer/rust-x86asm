use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 156, 224], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1412726661, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 156, 180, 79, 133, 127, 52, 84], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 156, 240], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 113638300, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 156, 164, 198, 156, 251, 197, 6], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 156, 224], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ESI, 1651499724, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 156, 158, 204, 226, 111, 98], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 156, 227], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1026548736, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 156, 148, 208, 0, 228, 47, 61], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 156, 208], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1879461258, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 156, 156, 126, 138, 77, 6, 112], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 159, 156, 60, 94], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 45, 133, 156, 199], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 69, 138, 156, 60, 222], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 93, 153, 156, 46], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 174, 156, 209], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 317471403, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 156, 28, 253, 171, 58, 236, 18], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 156, 60, 242], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 117, 172, 156, 223], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 780427963, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 109, 165, 156, 132, 210, 187, 98, 132, 46], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RBX, 1074213227, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 117, 178, 156, 171, 107, 49, 7, 64], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 254, 156, 236], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 156, 44, 95], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 223, 156, 31], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 5, 215, 156, 242], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM17)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 117, 193, 156, 62], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 989739985, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 117, 218, 156, 60, 117, 209, 59, 254, 58], OperandSize::Qword)
}

