use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 184, 215], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 184, 7], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 184, 231], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 184, 57], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 184, 232], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 91840484, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 184, 148, 83, 228, 95, 121, 5], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 184, 241], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 184, 3], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 184, 207], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 143502167, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 184, 164, 206, 87, 171, 141, 8], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 582051722, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 153, 184, 180, 151, 138, 103, 177, 34], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 93, 138, 184, 197], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 53, 135, 184, 44, 88], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 45, 153, 184, 43], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 174, 184, 223], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 170, 184, 7], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 190, 184, 60, 88], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 77, 167, 184, 239], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM10)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 45, 174, 184, 47], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 53, 187, 184, 28, 113], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 249, 184, 215], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1014870145, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 204, 184, 36, 69, 129, 176, 125, 60], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 221, 184, 8], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 109, 247, 184, 247], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 72952545, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 37, 198, 184, 52, 93, 225, 42, 89, 4], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 217, 184, 12, 199], OperandSize::Qword)
}

