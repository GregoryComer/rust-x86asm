use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 152, 192], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1042400200, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 152, 4, 205, 200, 195, 33, 62], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 152, 244], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 152, 56], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 152, 207], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 570734707, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 152, 132, 138, 115, 184, 4, 34], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 152, 219], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 152, 50], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 152, 197], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 544778565, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 137, 152, 156, 118, 69, 169, 120, 32], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 153524950, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 158, 152, 188, 119, 214, 154, 38, 9], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 221, 143, 152, 231], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1265758926, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 139, 152, 140, 193, 206, 242, 113, 75], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 253, 157, 152, 4, 247], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 173, 152, 247], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 174, 152, 60, 241], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 400526045, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 185, 152, 139, 221, 138, 223, 23], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 197, 164, 152, 226], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 560829739, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 189, 172, 152, 28, 125, 43, 149, 109, 33], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 832708187, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 190, 152, 132, 138, 91, 30, 162, 49], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 221, 152, 200], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 205, 152, 56], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 219, 152, 32], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 181, 253, 152, 247], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RDX, 2017873788, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 157, 199, 152, 130, 124, 79, 70, 120], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectDisplaced(RDI, 237024057, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 173, 223, 152, 151, 57, 179, 32, 14], OperandSize::Qword)
}

