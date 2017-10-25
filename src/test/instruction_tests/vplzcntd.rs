use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vplzcntd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 68, 193], OperandSize::Dword)
}

fn vplzcntd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 68, 47], OperandSize::Dword)
}

fn vplzcntd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 68, 22], OperandSize::Dword)
}

fn vplzcntd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 125, 138, 68, 214], OperandSize::Qword)
}

fn vplzcntd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM9)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 139, 68, 14], OperandSize::Qword)
}

fn vplzcntd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 156, 68, 63], OperandSize::Qword)
}

fn vplzcntd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 68, 221], OperandSize::Dword)
}

fn vplzcntd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 68, 20, 112], OperandSize::Dword)
}

fn vplzcntd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 68, 20, 74], OperandSize::Dword)
}

fn vplzcntd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 125, 172, 68, 197], OperandSize::Qword)
}

fn vplzcntd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM24)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 170, 68, 4, 200], OperandSize::Qword)
}

fn vplzcntd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(YMM16)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 185, 68, 0], OperandSize::Qword)
}

fn vplzcntd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 68, 241], OperandSize::Dword)
}

fn vplzcntd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 373923664, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 68, 140, 94, 80, 159, 73, 22], OperandSize::Dword)
}

fn vplzcntd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 68, 12, 208], OperandSize::Dword)
}

fn vplzcntd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 125, 204, 68, 210], OperandSize::Qword)
}

fn vplzcntd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM15)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 202, 68, 63], OperandSize::Qword)
}

fn vplzcntd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTD, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 220, 68, 12, 215], OperandSize::Qword)
}

