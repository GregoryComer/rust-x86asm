use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 188, 210], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1372599198, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 188, 156, 152, 158, 51, 208, 81], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 188, 199], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 188, 20, 193], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 188, 218], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 188, 60, 73], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 188, 200], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 188, 52, 87], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 188, 218], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 140, 188, 28, 122], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 153, 188, 14], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 37, 131, 188, 212], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 24421816, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 37, 139, 188, 44, 93, 184, 165, 116, 1], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 93, 156, 188, 14], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 188, 254], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 935006283, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 169, 188, 60, 141, 75, 16, 187, 55], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 187, 188, 60, 177], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 101, 167, 188, 205], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RDI, 100154109, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 109, 171, 188, 183, 253, 58, 248, 5], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 45, 181, 188, 52, 194], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 187, 188, 211], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EDX, 763314356, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 201, 188, 154, 180, 64, 127, 45], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EAX, 680102438, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 218, 188, 144, 38, 138, 137, 40], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 29, 189, 188, 200], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 101, 199, 188, 60, 81], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 209, 188, 36, 94], OperandSize::Qword)
}

