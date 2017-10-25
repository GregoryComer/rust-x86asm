use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrcp14pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 76, 250], OperandSize::Dword)
}

fn vrcp14pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 76, 23], OperandSize::Dword)
}

fn vrcp14pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EBX, 2127334739, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 76, 155, 83, 141, 204, 126], OperandSize::Dword)
}

fn vrcp14pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 253, 137, 76, 234], OperandSize::Qword)
}

fn vrcp14pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM19)), operand2: Some(IndirectDisplaced(RSI, 457000158, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 253, 141, 76, 158, 222, 68, 61, 27], OperandSize::Qword)
}

fn vrcp14pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 131956159, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 253, 158, 76, 148, 198, 191, 125, 221, 7], OperandSize::Qword)
}

fn vrcp14pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 76, 248], OperandSize::Dword)
}

fn vrcp14pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1146343298, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 76, 28, 245, 130, 207, 83, 68], OperandSize::Dword)
}

fn vrcp14pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 188, 76, 44, 82], OperandSize::Dword)
}

fn vrcp14pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 253, 174, 76, 210], OperandSize::Qword)
}

fn vrcp14pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1753400206, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 253, 170, 76, 180, 86, 142, 195, 130, 104], OperandSize::Qword)
}

fn vrcp14pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1580892287, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 253, 191, 76, 164, 201, 127, 128, 58, 94], OperandSize::Qword)
}

fn vrcp14pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 76, 192], OperandSize::Dword)
}

fn vrcp14pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 76, 44, 78], OperandSize::Dword)
}

fn vrcp14pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 223, 76, 12, 209], OperandSize::Dword)
}

fn vrcp14pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 253, 203, 76, 200], OperandSize::Qword)
}

fn vrcp14pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 992512240, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 76, 164, 123, 240, 136, 40, 59], OperandSize::Qword)
}

fn vrcp14pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 519187279, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 219, 76, 60, 213, 79, 43, 242, 30], OperandSize::Qword)
}

