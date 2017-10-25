use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 156, 195], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 1293456539, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 156, 137, 155, 148, 24, 77], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 156, 229], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 572476937, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 156, 12, 117, 9, 78, 31, 34], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 156, 195], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ESI, 1507079914, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 156, 142, 234, 54, 212, 89], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 156, 192], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RSI, 12587757, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 156, 166, 237, 18, 192, 0], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 138, 156, 248], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 156, 15], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1439855222, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 157, 156, 148, 134, 118, 114, 210, 85], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 77, 130, 156, 210], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM22)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 135, 156, 11], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 2128142226, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 53, 151, 156, 140, 123, 146, 223, 216, 126], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 170, 156, 228], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 156, 59], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 186, 156, 4, 178], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 85, 170, 156, 217], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 171, 156, 44, 178], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 53, 185, 156, 47], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 251, 156, 210], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 202, 156, 60, 199], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 218, 156, 39], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 53, 148, 156, 242], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RBX, 615671354, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 85, 205, 156, 139, 58, 102, 178, 36], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 308014341, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 214, 156, 132, 119, 5, 237, 91, 18], OperandSize::Qword)
}

