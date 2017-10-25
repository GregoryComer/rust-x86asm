use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtps2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 237], OperandSize::Dword)
}

fn vcvtps2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 366679153, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 4, 149, 113, 20, 219, 21], OperandSize::Dword)
}

fn vcvtps2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 234], OperandSize::Qword)
}

fn vcvtps2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDI, 1651023078, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 90, 143, 230, 156, 104, 98], OperandSize::Qword)
}

fn vcvtps2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 204], OperandSize::Dword)
}

fn vcvtps2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 6], OperandSize::Dword)
}

fn vcvtps2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 199], OperandSize::Qword)
}

fn vcvtps2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 417064086, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 90, 20, 125, 150, 228, 219, 24], OperandSize::Qword)
}

fn vcvtps2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 90, 219], OperandSize::Dword)
}

fn vcvtps2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 90, 60, 255], OperandSize::Dword)
}

fn vcvtps2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 124, 139, 90, 220], OperandSize::Qword)
}

fn vcvtps2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(XMM13)), operand2: Some(IndirectDisplaced(RSI, 294309390, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 124, 141, 90, 174, 14, 206, 138, 17], OperandSize::Qword)
}

fn vcvtps2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 90, 208], OperandSize::Dword)
}

fn vcvtps2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 90, 12, 67], OperandSize::Dword)
}

fn vcvtps2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 124, 174, 90, 213], OperandSize::Qword)
}

fn vcvtps2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1070433448, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 124, 170, 90, 60, 85, 168, 132, 205, 63], OperandSize::Qword)
}

fn vcvtps2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 157, 90, 197], OperandSize::Dword)
}

fn vcvtps2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 203, 90, 27], OperandSize::Dword)
}

fn vcvtps2pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 124, 156, 90, 237], OperandSize::Qword)
}

fn vcvtps2pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PD, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectDisplaced(RDX, 732762380, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 124, 203, 90, 170, 12, 17, 173, 43], OperandSize::Qword)
}

