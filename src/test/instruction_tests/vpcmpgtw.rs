use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpgtw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 101, 229], OperandSize::Dword)
}

fn vpcmpgtw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 101, 7], OperandSize::Dword)
}

fn vpcmpgtw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 101, 250], OperandSize::Qword)
}

fn vpcmpgtw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1340486030, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 101, 132, 82, 142, 49, 230, 79], OperandSize::Qword)
}

fn vpcmpgtw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 101, 244], OperandSize::Dword)
}

fn vpcmpgtw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 789118305, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 101, 20, 189, 97, 253, 8, 47], OperandSize::Dword)
}

fn vpcmpgtw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 101, 225], OperandSize::Qword)
}

fn vpcmpgtw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RBX, 1439538764, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 101, 187, 76, 158, 205, 85], OperandSize::Qword)
}

fn vpcmpgtw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 15, 101, 202], OperandSize::Dword)
}

fn vpcmpgtw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1686663324, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 15, 101, 44, 189, 156, 112, 136, 100], OperandSize::Dword)
}

fn vpcmpgtw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 29, 11, 101, 215], OperandSize::Qword)
}

fn vpcmpgtw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 1, 101, 44, 142], OperandSize::Qword)
}

fn vpcmpgtw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 45, 101, 218], OperandSize::Dword)
}

fn vpcmpgtw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 995193770, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 44, 101, 140, 126, 170, 115, 81, 59], OperandSize::Dword)
}

fn vpcmpgtw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 21, 35, 101, 204], OperandSize::Qword)
}

fn vpcmpgtw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 466836431, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 29, 34, 101, 172, 137, 207, 91, 211, 27], OperandSize::Qword)
}

fn vpcmpgtw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 75, 101, 240], OperandSize::Dword)
}

fn vpcmpgtw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 78, 101, 8], OperandSize::Dword)
}

fn vpcmpgtw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 101, 76, 101, 254], OperandSize::Qword)
}

fn vpcmpgtw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RDX, 298168796, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 37, 75, 101, 162, 220, 177, 197, 17], OperandSize::Qword)
}

