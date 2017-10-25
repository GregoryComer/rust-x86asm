use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 188, 236], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 1868414977, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 188, 156, 128, 1, 192, 93, 111], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 188, 219], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1772485930, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 188, 148, 155, 42, 253, 165, 105], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 188, 243], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 311146142, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 188, 4, 93, 158, 182, 139, 18], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 188, 213], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDI, 393281290, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 188, 135, 10, 255, 112, 23], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 139, 188, 210], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 2057987268, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 188, 132, 135, 196, 100, 170, 122], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 156, 188, 44, 67], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 173, 132, 188, 255], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1598161613, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 181, 138, 188, 132, 201, 205, 2, 66, 95], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1744491186, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 157, 159, 188, 164, 246, 178, 210, 250, 103], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 174, 188, 254], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 188, 7], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EBX, 481271625, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 186, 188, 131, 73, 159, 175, 28], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 245, 166, 188, 215], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 2092711512, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 149, 170, 188, 180, 186, 88, 62, 188, 124], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 314603355, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 229, 178, 188, 156, 137, 91, 119, 192, 18], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 223, 188, 240], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1574719365, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 188, 36, 205, 133, 79, 220, 93], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1248776049, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 222, 188, 28, 117, 113, 207, 110, 74], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 189, 215, 188, 239], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RCX, 1722401340, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 197, 204, 188, 185, 60, 194, 169, 102], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 165, 212, 188, 41], OperandSize::Qword)
}

