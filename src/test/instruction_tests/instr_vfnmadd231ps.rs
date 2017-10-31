use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 188, 210], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 733412416, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 188, 187, 64, 252, 182, 43], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 188, 246], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 649705290, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 188, 4, 221, 74, 183, 185, 38], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 188, 208], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 188, 20, 248], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 188, 229], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1220673135, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 188, 156, 153, 111, 254, 193, 72], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 188, 232], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 188, 35], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1288955068, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 158, 188, 172, 151, 188, 228, 211, 76], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 29, 142, 188, 239], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RBX, 1488639967, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 135, 188, 147, 223, 215, 186, 88], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 155101426, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 117, 159, 188, 44, 189, 242, 168, 62, 9], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 188, 206], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1921971471, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 173, 188, 20, 197, 15, 245, 142, 114], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EAX, 2091623275, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 190, 188, 176, 107, 163, 171, 124], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 61, 166, 188, 253], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RAX, 1111927082, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 77, 172, 188, 144, 42, 169, 70, 66], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 101, 191, 188, 63], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 190, 188, 217], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1255455266, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 207, 188, 20, 197, 34, 186, 212, 74], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 216976406, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 219, 188, 172, 210, 22, 204, 238, 12], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 45, 254, 188, 247], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 199, 188, 60, 126], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 13, 215, 188, 12, 223], OperandSize::Qword)
}

