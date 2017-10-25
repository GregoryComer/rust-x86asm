use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovsxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 200], OperandSize::Dword)
}

fn vpmovsxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1034754535, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 188, 179, 231, 25, 173, 61], OperandSize::Dword)
}

fn vpmovsxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 201], OperandSize::Qword)
}

fn vpmovsxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 55], OperandSize::Qword)
}

fn vpmovsxbd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 218], OperandSize::Dword)
}

fn vpmovsxbd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EAX, 1874419564, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 160, 108, 95, 185, 111], OperandSize::Dword)
}

fn vpmovsxbd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 202], OperandSize::Qword)
}

fn vpmovsxbd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 12, 203], OperandSize::Qword)
}

fn vpmovsxbd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 33, 216], OperandSize::Dword)
}

fn vpmovsxbd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 731910370, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 33, 156, 192, 226, 16, 160, 43], OperandSize::Dword)
}

fn vpmovsxbd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 125, 137, 33, 214], OperandSize::Qword)
}

fn vpmovsxbd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 416170552, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 137, 33, 60, 221, 56, 66, 206, 24], OperandSize::Qword)
}

fn vpmovsxbd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 33, 209], OperandSize::Dword)
}

fn vpmovsxbd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 713737887, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 33, 132, 186, 159, 198, 138, 42], OperandSize::Dword)
}

fn vpmovsxbd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 125, 175, 33, 197], OperandSize::Qword)
}

fn vpmovsxbd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM29)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 171, 33, 47], OperandSize::Qword)
}

fn vpmovsxbd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 33, 238], OperandSize::Dword)
}

fn vpmovsxbd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1689936762, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 33, 156, 115, 122, 99, 186, 100], OperandSize::Dword)
}

fn vpmovsxbd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 125, 202, 33, 245], OperandSize::Qword)
}

fn vpmovsxbd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM28)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 87843998, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 207, 33, 36, 117, 158, 100, 60, 5], OperandSize::Qword)
}

